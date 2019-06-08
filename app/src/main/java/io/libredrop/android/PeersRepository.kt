package io.libredrop.android

import android.util.Log
import androidx.lifecycle.LiveData
import androidx.lifecycle.MutableLiveData
import io.libredrop.network.Network
import io.libredrop.network.PeerInfo
import kotlinx.coroutines.CoroutineScope
import kotlinx.coroutines.Job
import kotlinx.coroutines.asCoroutineDispatcher
import kotlinx.coroutines.channels.actor
import kotlinx.coroutines.launch
import java.util.concurrent.Executors
import kotlin.coroutines.CoroutineContext
import kotlin.properties.Delegates

class PeersRepository : CoroutineScope {
    private val job = Job()
    override val coroutineContext: CoroutineContext
        get() = Executors.newSingleThreadExecutor().asCoroutineDispatcher() + job

    private val network = actor<Action> {
        val network = Network(::onNewConnectionFound)
        network.startDiscovery()

        Log.i(TAG, "Network is active")

        for (action in channel) {
            when (action) {
                is Action.SendMessage -> network.sendMessage(action.peerInfo, action.message)
            }
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

    fun sendMessage(peerInfo: PeerInfo, message: String) {
        launch {
            network.send(Action.SendMessage(peerInfo, message))
        }
    }

    private sealed class Action {
        class SendMessage(val peerInfo: PeerInfo, val message: String) : Action()
    }

    companion object {
        private val TAG = PeersRepository::class.java.canonicalName
    }
}
