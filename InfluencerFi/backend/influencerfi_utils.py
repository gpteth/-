# Utility functions for interacting with smart contracts and handling transactions
from web3 import Web3
from web3.middleware import geth_poa_middleware
from dotenv import load_dotenv
import os

load_dotenv()

w3 = Web3(Web3.HTTPProvider(os.getenv("WEB3_PROVIDER_URL")))
w3.middleware_onion.inject(geth_poa_middleware, layer=0)

def deploy_contract(abi, bytecode, sender_address, private_key):
    # Deploy a smart contract
    contract = w3.eth.contract(abi=abi, bytecode=bytecode)
    transaction = contract.constructor().buildTransaction({
        'chainId': 1,  # Replace with the appropriate chain ID
        'gas': 2000000,
        'gasPrice': w3.toWei('50', 'gwei'),
        'nonce': w3.eth.getTransactionCount(sender_address),
    })
    signed_transaction = w3.eth.account.signTransaction(transaction, private_key)
    tx_hash = w3.eth.sendRawTransaction(signed_transaction.rawTransaction)
    tx_receipt = w3.eth.waitForTransactionReceipt(tx_hash)
    return tx_receipt.contractAddress

# Other utility functions for contract interactions
