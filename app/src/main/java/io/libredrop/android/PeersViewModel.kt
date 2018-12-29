package io.libredrop.android

import androidx.lifecycle.LiveData
import androidx.lifecycle.ViewModel
import io.libredrop.network.PeerInfo

class PeersViewModel : ViewModel() {

    private val peersRepository = PeersRepository()

    val peers: LiveData<List<PeerInfo>>
        get() = peersRepository.peers
}
