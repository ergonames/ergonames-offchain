"use strict";
var __awaiter = (this && this.__awaiter) || function (thisArg, _arguments, P, generator) {
    function adopt(value) { return value instanceof P ? value : new P(function (resolve) { resolve(value); }); }
    return new (P || (P = Promise))(function (resolve, reject) {
        function fulfilled(value) { try { step(generator.next(value)); } catch (e) { reject(e); } }
        function rejected(value) { try { step(generator["throw"](value)); } catch (e) { reject(e); } }
        function step(result) { result.done ? resolve(result.value) : adopt(result.value).then(fulfilled, rejected); }
        step((generator = generator.apply(thisArg, _arguments || [])).next());
    });
};
var __importDefault = (this && this.__importDefault) || function (mod) {
    return (mod && mod.__esModule) ? mod : { "default": mod };
};
Object.defineProperty(exports, "__esModule", { value: true });
const axios_1 = __importDefault(require("axios"));
const client_1 = require("@prisma/client");
const prisma = new client_1.PrismaClient();
const DATABASE_PATH = "postgresql://ergonames:ergonames@localhost:5432/ergonames";
const INDEXED_NODE_URL = "http://198.58.96.195:9052";
const INITIAL_AVLTREE_CREATION_TRANSACTION = "e271e7cb9b9c7932546e8a5746c91cb1c0f1114ff173a90e1fe979170f71c579";
const PROXY_CONTRACT_ERGOTREE = "1003040001010400d801d601b2a5730000d1eded7301e6c67201040e93c17201c1b2a4730200";
const getInitialTransactionInformation = (initialTransactionId = INITIAL_AVLTREE_CREATION_TRANSACTION) => __awaiter(void 0, void 0, void 0, function* () {
    let url = `${INDEXED_NODE_URL}/blockchain/transaction/byId/${initialTransactionId}`;
    let response = yield axios_1.default.get(url);
    let transaction = response.data;
    let transactionId = transaction.id;
    let boxId = transaction.outputs[0].boxId;
    let spentTransactionId = transaction.outputs[0].spentTransactionId;
    let initialTransactionInformation = {
        transactionId: transactionId,
        boxId: boxId,
        spentTransactionId: spentTransactionId
    };
    return initialTransactionInformation;
});
const getMintInformation = (lastSpentTransactionId) => __awaiter(void 0, void 0, void 0, function* () {
    let url = `${INDEXED_NODE_URL}/blockchain/transaction/byId/${lastSpentTransactionId}`;
    let response = yield axios_1.default.get(url);
    let body = response.data;
    let ergonameRaw = body.outputs[0].additionalRegisters.R4;
    ergonameRaw = ergonameRaw.substring(4);
    let ergonameRegistered = Buffer.from(ergonameRaw, "hex").toString("utf8");
    let mintTransactionId = body.id;
    let mintBoxId = body.outputs[0].boxId;
    let spendTransactionId = body.outputs[1].spentTransactionId;
    let ergonameTokenId = body.outputs[0].assets[0].tokenId;
    let registrationInformation = {
        ergonameRegistered: ergonameRegistered,
        mintTransactionId: mintTransactionId,
        mintBoxId: mintBoxId,
        spendTransactionId: spendTransactionId,
        ergonameTokenId: ergonameTokenId
    };
    return registrationInformation;
});
const getMintRequestAtProxyAddress = (proxyAddressErgoTree = PROXY_CONTRACT_ERGOTREE) => __awaiter(void 0, void 0, void 0, function* () {
    let url = `${INDEXED_NODE_URL}/blockchain/box/unspent/byErgoTree`;
    let response = yield axios_1.default.post(url, proxyAddressErgoTree);
    let transactionsData = response.data;
    let transactions = [];
    for (let tx of transactionsData) {
        let transactionId = tx.transactionId;
        let boxId = tx.boxId;
        let mintRequest = {
            boxId: boxId,
            transactionId: transactionId
        };
        transactions.push(mintRequest);
    }
    return transactions;
});
const writeToConfirmedRegistryInsertions = (ergoname) => __awaiter(void 0, void 0, void 0, function* () {
    yield prisma.confirmed_registry_insertions.create({
        data: {
            ergoname_registered: ergoname.ergonameRegistered,
            ergoname_token_id: ergoname.ergonameTokenId,
            mint_box_id: ergoname.mintBoxId,
            mint_transaction_id: ergoname.mintTransactionId,
            spend_transaction_id: ergoname.spendTransactionId
        }
    });
});
const writeToMintRequests = (mintRequest) => __awaiter(void 0, void 0, void 0, function* () {
    yield prisma.mint_requests.upsert({
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
    });
});
const syncInitialRegistry = () => __awaiter(void 0, void 0, void 0, function* () {
    let initialTransactionInformation = yield getInitialTransactionInformation();
    let registrationInformation = yield getMintInformation(initialTransactionInformation.spentTransactionId);
    yield writeToConfirmedRegistryInsertions(registrationInformation);
    let lastSpentTransactionId = registrationInformation.spendTransactionId;
    while (lastSpentTransactionId != null) {
        let registrationInformation = yield getMintInformation(lastSpentTransactionId);
        yield writeToConfirmedRegistryInsertions(registrationInformation);
        lastSpentTransactionId = registrationInformation.spendTransactionId;
    }
});
const continuousSync = () => __awaiter(void 0, void 0, void 0, function* () {
    let lastRegistryInsertion = yield prisma.$queryRaw `SELECT * FROM confirmed_registry_insertions WHERE spend_transaction_id IS NULL`;
    let lastSpentTransactionId = lastRegistryInsertion.spendTransactionId;
    if (lastSpentTransactionId != null) {
        let registrationInformation = yield getMintInformation(lastSpentTransactionId);
        while (registrationInformation.spendTransactionId != null) {
            yield writeToConfirmedRegistryInsertions(registrationInformation);
            registrationInformation = yield getMintInformation(registrationInformation.spendTransactionId);
        }
    }
});
const trackMintRequests = () => __awaiter(void 0, void 0, void 0, function* () {
    let mintRequests = yield getMintRequestAtProxyAddress();
    for (let mintRequest of mintRequests) {
        yield writeToMintRequests(mintRequest);
    }
});
const checkIfRegistryTableIsEmpty = () => __awaiter(void 0, void 0, void 0, function* () {
    let count = yield prisma.confirmed_registry_insertions.count();
    return count > 0;
});
const main = () => __awaiter(void 0, void 0, void 0, function* () {
    let registryTableIsEmpty = yield checkIfRegistryTableIsEmpty();
    if (!registryTableIsEmpty) {
        yield syncInitialRegistry();
    }
    while (true) {
        yield continuousSync();
        yield trackMintRequests();
    }
    // let mintRequest: Array<MintRequest> = await getMintRequestAtProxyAddress();
    // console.log(mintRequest);
    // const allErgonames = await prisma.confirmed_registry_insertions.findMany();
    // console.log(allErgonames);
});
main()
    .then(() => __awaiter(void 0, void 0, void 0, function* () {
    yield prisma.$disconnect();
}))
    .catch((e) => __awaiter(void 0, void 0, void 0, function* () {
    console.error(e);
    yield prisma.$disconnect();
    process.exit(1);
}));
