package io.libredrop.android

import androidx.lifecycle.LiveData
import androidx.lifecycle.MutableLiveData
import io.libredrop.network.Network
import io.libredrop.network.PeerInfo
import kotlin.concurrent.thread
import kotlin.properties.Delegates

class PeersRepository {
    private val network = Network(::onNewConnectionFound)

    init {
        thread {
            network.startDiscovery()
        }
    }

    private var allPeers: List<PeerInfo> by Delegates.observable(emptyList()) { _, _, new ->
        _peers.postValue(new)
    }

    private val _peers = MutableLiveData<List<PeerInfo>>()
    val peers: LiveData<List<PeerInfo>> = _peers

    private fun onNewConnectionFound(peerInfo: PeerInfo) {
        allPeers += peerInfo
    }
}
