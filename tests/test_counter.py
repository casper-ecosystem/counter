import pytest
import requests
import casperlabs_client
import json
from typing import List

import unittest
from test.cl_test_context import NetworkInstance
from test.cl_node.casperlabs_accounts import Account, GENESIS_ACCOUNT

URL = 'http://localhost:40403/graphql'

def deploy_and_propose_from_genesis(node, contract, account):
    # node.use_python_client()
    return node.deploy_and_propose(
        session_contract=contract,
        payment_contract=contract,
        from_address=account.public_key_hex,
        public_key=account.public_key_path,
        private_key=account.private_key_path,
    )

def pp_json(json_thing, sort=True, indents=4):
    try:
        if type(json_thing) is str:
            print(json.dumps(json.loads(json_thing), sort_keys=sort, indent=indents))
        else:
            print(json.dumps(json_thing, sort_keys=sort, indent=indents))
    except Exception:
        print(json_thing)
    return None

def query(graph_ql_query: str, debug: bool = False) -> dict:
    query_json = {"query": graph_ql_query}
    r = requests.post(url=URL, json=query_json)
    if debug:
        print(r.text)
    return json.loads(r.text)['data']

def query_global_state(block_hash: str, key_type:str, key:str, path_segments:List[str]) -> dict:
    q = f"""
        query {{
          globalState(
            blockHashBase16Prefix: "{block_hash}"
            StateQueries: [
              {{
                keyType: {key_type}
                keyBase16: "{key}"
                # Change single quote to double quote. 
                pathSegments: {str(path_segments).replace("'", '"') }
              }}
            ]
          ) {{
            value {{
              __typename
              ... on IntValue {{
                int: value
              }}
              ... on ByteString {{
                bytes: value
              }}
              ... on IntList {{
                ints: value
              }}
              ... on StringValue {{
                string: value
              }}
              ... on Account {{
                pubKey
                purseId {{
                  uref
                  accessRights
                }}
                knownUrefs {{
                  name
                  key {{
                    ...KeyFields
                  }}
                }}
                associatedKeys {{
                  pubKey
                  weight
                }}
                actionThreshold {{
                  deploymentThreshold
                  keyManagementThreshold
                }}
                accountActivity {{
                  keyManagementLastUsed
                  deploymentLastUsed
                  inactivityPeriodLimit
                }}
              }}
              ... on Contract {{
                knownUrefs {{
                  name
                  key {{
                    ...KeyFields
                  }}
                }}
                protocolVersion
              }}
              ... on StringList {{
                strings: value
              }}
              ... on NamedKey {{
                name
                key {{
                  ...KeyFields
                }}
              }}
              ... on BigIntValue {{
                big_int_value: value
                bitWidth
              }}
              ... on Key {{
                ...KeyFields
              }}
              ... on Unit {{
                unit: value
              }}
            }}
          }}
        }}

        fragment KeyFields on Key {{
          value {{
            __typename
            ... on KeyAddress {{
              key_address: value
            }}
            ... on KeyHash {{
              key_hash: value
            }}
            ... on KeyUref {{
              uref
              accessRights
            }}
            ... on KeyLocal {{
              hash
            }}
          }}
        }}
    """
    return query(q)['globalState']

def get_latest_block_hash() -> str:
    q = f"""{{
            dagSlice(depth: 1) {{
                blockHash
            }}
        }}"""
    response = query(q)
    return response['dagSlice'][0]['blockHash']

def get_count_value(account: Account, block_hash: str = None) -> dict:
    block_hash = block_hash or get_latest_block_hash()
    
    return query_global_state(
        block_hash, 
        "Address", 
        account.public_key_hex, 
        ["counter", "count"]
    )[0]['value']['int']


class TestCounter(unittest.TestCase):

    @unittest.skip("Skiped")
    def test_counter(self):
        network = NetworkInstance()
        node: DockerNode = network.docker_nodes[0]

        # # Prepare accounts with balances
        account: Account = network.get_key()
        block_hash = node.transfer_to_account(account.file_id, 100)

        # Deploy counter smart contract and check initial value.
        # block_hash = deploy_and_propose_from_genesis(node, "test_counterdefine.wasm", account)
        block_hash = deploy_and_propose_from_genesis(node, "counter_define.wasm", account)
        self.assertEqual(get_count_value(account, block_hash), 0)

        # Increment value
        block_hash = deploy_and_propose_from_genesis(node, "counter_call.wasm", account)
        self.assertEqual(get_count_value(account, block_hash), 1)

    def test_counter2(self):
        network = NetworkInstance()
        node: DockerNode = network.docker_nodes[0]

        # # Prepare accounts with balances
        account: Account = network.get_key()
        block_hash = node.transfer_to_account(account.file_id, 100)

        # Deploy counter smart contract and check initial value.
        # block_hash = deploy_and_propose_from_genesis(node, "test_counterdefine.wasm", account)
        block_hash = deploy_and_propose_from_genesis(node, "counter_define.wasm", account)
        self.assertEqual(get_count_value(account, block_hash), 0)

        # Increment value
        block_hash = deploy_and_propose_from_genesis(node, "counter_call.wasm", account)
        self.assertEqual(get_count_value(account, block_hash), 2)

        # # Increment value
        # block_hash = deploy_and_propose_from_genesis(node, "counter_call.wasm", account)
        # self.assertEqual(get_count_value(account, block_hash), 2)

        # # Increment value
        # block_hash = deploy_and_propose_from_genesis(node, "counter_call.wasm", account)
        # self.assertEqual(get_count_value(account, block_hash), 3)

        # # Increment value
        # block_hash = deploy_and_propose_from_genesis(node, "counter_call.wasm", account)
        # self.assertEqual(get_count_value(account, block_hash), 4)

        # # Increment value
        # block_hash = deploy_and_propose_from_genesis(node, "counter_call.wasm", account)
        # self.assertEqual(get_count_value(account, block_hash), 5)

        # # Increment value
        # block_hash = deploy_and_propose_from_genesis(node, "counter_call.wasm", account)
        # self.assertEqual(get_count_value(account, block_hash), 6)

        # # Increment value
        # block_hash = deploy_and_propose_from_genesis(node, "counter_call.wasm", account)
        # self.assertEqual(get_count_value(account, block_hash), 7)
