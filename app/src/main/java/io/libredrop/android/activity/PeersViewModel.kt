package io.libredrop.android.activity

import androidx.lifecycle.LiveData
import androidx.lifecycle.ViewModel
import io.libredrop.network.PeerInfo
import javax.inject.Inject

class PeersViewModel @Inject constructor() : ViewModel() {

    private val peersRepository = PeersRepository()

    val peers: LiveData<List<PeerInfo>>
        get() = peersRepository.peers

    fun onSendClick(peerInfo: PeerInfo) {
        peersRepository.sendMessage(peerInfo, "Foo bar")
    }
}
