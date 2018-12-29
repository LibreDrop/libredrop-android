package io.libredrop.network

class Network(val listener: (name: String) -> Unit) {
    external fun startDiscovery()

    external fun stopDiscovery()

    private fun onNewConnectionFound(name: String) {
        listener(name)
    }

    companion object {
        @JvmStatic
        private external fun init()

        init {
            System.loadLibrary("rust")
            init()
        }
    }
}
