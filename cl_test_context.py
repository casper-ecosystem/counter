class NetworkInstance():
    __network = None
    def __new__(cls, network=None):
        if NetworkInstance.__network is None:
            if network is None:
                raise RuntimeError("NetworkInstance needs to be initialized.")
            else:
                NetworkInstance.__network = network
        elif network is not None:
            raise RuntimeError("NetworkInstance is already initialized.")

        return NetworkInstance.__network

if __name__ == "__main__":
    import unittest
    tc = unittest.TestCase()

    # First use should take network as argument
    with tc.assertRaises(RuntimeError):
        nc = NetworkInstance()

    # Initialization works fine.
    my_object = "Hello World"
    NetworkInstance(my_object)

    # Can't initialize it again.
    second_object = "Long time ago..."
    with tc.assertRaises(RuntimeError):
        nc = NetworkInstance(second_object)

    # Nothing new is created.
    tc.assertEqual(NetworkInstance(), my_object)
    tc.assertEqual(NetworkInstance(), my_object)
