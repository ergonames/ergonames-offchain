import axios from "axios";
import { PrismaClient } from "@prisma/client";
const prisma = new PrismaClient();

const INDEXED_NODE_URL: string = "http://198.58.96.195:9052";
const INITIAL_AVLTREE_CREATION_TRANSACTION: string = "e271e7cb9b9c7932546e8a5746c91cb1c0f1114ff173a90e1fe979170f71c579";
const PROXY_CONTRACT_ERGOTREE: string = "1003040001010400d801d601b2a5730000d1eded7301e6c67201040e93c17201c1b2a4730200";

interface InitialTransactionInformation {
    boxId: string;
    transactionId: string;
    spentTransactionId: string;
}

interface RegistrationInformation {
    ergonameRegistered: string;
    ergonameTokenId: string;
    mintBoxId: string;
    mintTransactionId: string;
    spendTransactionId: string;
}

interface MintRequest {
    boxId: string;
    transactionId: string;
}

const getInitialTransactionInformation = async (initialTransactionId: string = INITIAL_AVLTREE_CREATION_TRANSACTION): Promise<InitialTransactionInformation> => {
    let url: string = `${INDEXED_NODE_URL}/blockchain/transaction/byId/${initialTransactionId}`;
    let response: any = await axios.get(url);
    let transaction: any = response.data;
    let transactionId: string = transaction.id;
    let boxId: string = transaction.outputs[0].boxId;
    let spentTransactionId: string = transaction.outputs[0].spentTransactionId;
    let initialTransactionInformation: InitialTransactionInformation = {
        transactionId: transactionId,
        boxId: boxId,
        spentTransactionId: spentTransactionId
    };
    return initialTransactionInformation;
}

const getMintInformation = async (lastSpentTransactionId: string): Promise<RegistrationInformation> => {
    let url: string = `${INDEXED_NODE_URL}/blockchain/transaction/byId/${lastSpentTransactionId}`;
    let response: any = await axios.get(url);
    let body: any = response.data;
    let ergonameRaw: string = body.outputs[0].additionalRegisters.R4;
    ergonameRaw = ergonameRaw.substring(4);
    let ergonameRegistered: string = Buffer.from(ergonameRaw, "hex").toString("utf8");
    let mintTransactionId: string = body.id;
    let mintBoxId: string = body.outputs[0].boxId;
    let spendTransactionId: string = body.outputs[1].spentTransactionId;
    let ergonameTokenId: string = body.outputs[0].assets[0].tokenId;
    let registrationInformation: RegistrationInformation = {
        ergonameRegistered: ergonameRegistered,
        mintTransactionId: mintTransactionId,
        mintBoxId: mintBoxId,
        spendTransactionId: spendTransactionId,
        ergonameTokenId: ergonameTokenId
    };
    return registrationInformation;
}

const getMintRequestAtProxyAddress = async (proxyAddressErgoTree: string = PROXY_CONTRACT_ERGOTREE): Promise<Array<MintRequest>> => {
    let url: string = `${INDEXED_NODE_URL}/blockchain/box/unspent/byErgoTree`;
    let response: any = await axios.post(url, proxyAddressErgoTree);
    let transactionsData: any = response.data;
    let transactions: Array<MintRequest> = [];
    for (let tx of transactionsData) {
        let transactionId: string = tx.transactionId;
        let boxId: string = tx.boxId;
        let mintRequest: MintRequest = {
            boxId: boxId,
            transactionId: transactionId
        };
        transactions.push(mintRequest);
    }
    return transactions;
}

const writeToConfirmedRegistryInsertions = async (ergoname: RegistrationInformation) => {
    await prisma.confirmed_registry_insertions.create({
        data: {
            ergoname_registered: ergoname.ergonameRegistered,
            ergoname_token_id: ergoname.ergonameTokenId,
            mint_box_id: ergoname.mintBoxId,
            mint_transaction_id: ergoname.mintTransactionId,
            spend_transaction_id: ergoname.spendTransactionId
        }
    });
}

const writeToMintRequests = async (mintRequest: MintRequest) => {
    await prisma.mint_requests.upsert({
        where: {
            box_id: mintRequest.boxId
        },
        create: {
            box_id: mintRequest.boxId,
            transaction_id: mintRequest.transactionId
        },
        update: {
            transaction_id: mintRequest.transactionId
        }
    })
}

const syncInitialRegistry = async () => {
    let initialTransactionInformation: InitialTransactionInformation = await getInitialTransactionInformation();
    let registrationInformation: RegistrationInformation = await getMintInformation(initialTransactionInformation.spentTransactionId);
    await writeToConfirmedRegistryInsertions(registrationInformation);
    let lastSpentTransactionId: string = registrationInformation.spendTransactionId;
    while (lastSpentTransactionId != null) {
        let registrationInformation: RegistrationInformation = await getMintInformation(lastSpentTransactionId);
        await writeToConfirmedRegistryInsertions(registrationInformation);
        lastSpentTransactionId = registrationInformation.spendTransactionId;
    }
}

const continuousSync = async () => {
    let lastRegistryInsertion: RegistrationInformation = await prisma.$queryRaw`SELECT * FROM confirmed_registry_insertions WHERE spend_transaction_id IS NULL`;
    let lastSpentTransactionId: string = lastRegistryInsertion.spendTransactionId;
    if (lastSpentTransactionId != null) {
        let registrationInformation: RegistrationInformation = await getMintInformation(lastSpentTransactionId);
        while (registrationInformation.spendTransactionId != null) {
            await writeToConfirmedRegistryInsertions(registrationInformation);
            registrationInformation = await getMintInformation(registrationInformation.spendTransactionId);
        }
    }
}

const trackMintRequests = async () => {
    let mintRequests: Array<MintRequest> = await getMintRequestAtProxyAddress();
    for (let mintRequest of mintRequests) {
        await writeToMintRequests(mintRequest);
    }
}

const checkIfRegistryTableIsEmpty = async () => {
    let count: number = await prisma.confirmed_registry_insertions.count();
    return count > 0;
}

const main = async () => {
    let registryTableIsEmpty: boolean = await checkIfRegistryTableIsEmpty();
    if (!registryTableIsEmpty) {
        await syncInitialRegistry();
    }

    while (true) {
        await continuousSync();
        await trackMintRequests();
    }
}

main()
    .then(async () => {
        await prisma.$disconnect();
    })
    .catch(async (e) => {
        console.error(e);
        await prisma.$disconnect();
        process.exit(1);
    }
);
