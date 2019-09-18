import importlib
import os
import subprocess
import atexit
import unittest
import pickle
import docker as docker_py

from casperlabs_local_net.common import random_string, MAX_PAYMENT_COST
from casperlabs_local_net.casperlabs_network import CasperLabsNetwork
from casperlabs_local_net.docker_base import DockerConfig

from cl_test_context import NetworkInstance


class StandaloneNetwork(CasperLabsNetwork):
    """ A single node network with just a bootstrap """

    is_payment_code_enabled = False
    initial_motes = MAX_PAYMENT_COST * 100  # 10 millions * 100 = 1 billion motes
    grpc_encryption = False

    def create_cl_network(self):
        kp = self.get_key()
        config = DockerConfig(
            self.docker_client,
            node_private_key=kp.private_key,
            node_public_key=kp.public_key,
            network=self.create_docker_network(),
            initial_motes=self.initial_motes,
            node_account=kp,
            grpc_encryption=self.grpc_encryption,
            command_timeout=20
        )
        self.add_bootstrap(config)

print("Starting local network...")
docker_client = docker_py.from_env()
network = StandaloneNetwork(docker_client)
network.create_cl_network()
NetworkInstance(network)
print("Starting local network... DONE!")


def cargo(command: str):
    os.chdir("../../../")
    result = subprocess.call(["cargo", "make", command])
    os.chdir("./target/CasperLabs/integration-testing/")
    return result


def run_tests():
    result = cargo("reload")
    if result != 0:
        print("Build failed.")
        return

    files = os.listdir('test')
    modules_names = [f[:-3] for f in files if f.startswith("test_") and f.endswith(".py")]

    loader = unittest.TestLoader()
    suites = unittest.TestSuite()
    for module_name in modules_names:
        module = importlib.import_module(f"test.{module_name}")
        importlib.reload(module)
        module_tests = loader.loadTestsFromModule(module)
        suites.addTest(module_tests)

    print("\nTests:")

    runner = unittest.TextTestRunner(verbosity=2, failfast=True, buffer=True)
    runner.run(suites)

run_tests()

# def on_exit():
#     print("Stopping local network...")
#     docker_client.containers.prune()
#     docker_client.volumes.prune()
#     docker_client.networks.prune()
#     print("Stopping local network... DONE!")
# atexit.register(on_exit)
