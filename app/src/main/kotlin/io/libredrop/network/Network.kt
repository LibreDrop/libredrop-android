package io.libredrop.network

class Network(val listener: (peerInfo: PeerInfo) -> Unit) {
    external fun startDiscovery()

    external fun stopDiscovery()

    private fun onNewConnectionFound(peerInfo: PeerInfo) {
        listener(peerInfo)
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
