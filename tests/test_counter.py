import json
import unittest

from casperlabs_local_net.casperlabs_accounts import Account
from casperlabs_local_net.common import Contract

from cl_test_context import NetworkInstance, deploy_and_propose, query_global_state, next_account

class TestCounter(unittest.TestCase):

    # @unittest.skip("Skiped")
    def test_counter(self):
        network = NetworkInstance()
        
        # Prepare account with positive balance.
        account: Account = next_account(network, 100000000)

        # Deploy counter smart contract and check initial value.
        block_hash, deploy_hash = deploy_and_propose(network, "counter_define.wasm", account)
        self.assertEqual(get_count_value(account, block_hash), 0)

        # # Increment value
        block_hash, deploy_hash = deploy_and_propose(network, "counter_call.wasm", account)
        self.assertEqual(get_count_value(account, block_hash), 1)

        # # Increment value again.
        block_hash, deploy_hash = deploy_and_propose(network, "counter_call.wasm", account)
        self.assertEqual(get_count_value(account, block_hash), 2)


def get_count_value(account: Account, block_hash: str = None) -> int:
    block_hash = block_hash or get_latest_block_hash()
    return query_global_state(
        block_hash, 
        "Address", 
        account.public_key_hex, 
        ["counter", "count"]
    )[0]['value']['int']

