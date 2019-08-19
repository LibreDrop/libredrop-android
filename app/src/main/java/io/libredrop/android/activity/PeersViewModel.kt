package io.libredrop.android.activity

import android.util.Log
import androidx.lifecycle.LiveData
import io.libredrop.android.BaseViewModel
import io.libredrop.android.Navigation
import io.libredrop.network.PeerInfo
import kotlinx.coroutines.launch
import javax.inject.Inject

class PeersViewModel @Inject constructor(
    private val peersRepository: PeersRepository,
    private val navigation: Navigation
) : BaseViewModel() {

    val peers: LiveData<List<PeerInfo>>
        get() = peersRepository.peers

    fun onSendClick(peerInfo: PeerInfo) {
        peersRepository.sendMessage(peerInfo, "Foo bar")

        launch {
            val uri = navigation.openFileSelection()
            Log.d(javaClass.simpleName, "Open file: $uri")
        }
    }
}
