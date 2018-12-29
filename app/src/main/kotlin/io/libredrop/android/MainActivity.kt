package io.libredrop.android

import android.app.Activity
import android.os.Bundle
import android.os.Handler
import android.widget.Toast
import io.libredrop.network.Network
import io.libredrop.network.PeerInfo
import kotlin.concurrent.thread

class MainActivity : Activity() {

    private val network = Network(::onNewConnectionFound)
    private val handler = Handler();

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        setContentView(R.layout.activity_main)
    }

    override fun onStart() {
        super.onStart()

        thread {
            network.startDiscovery()
        }
    }

    override fun onStop() {
        super.onStop()

        network.stopDiscovery()
    }

    private fun onNewConnectionFound(peerInfo: PeerInfo) {
        handler.post {
            Toast.makeText(this, "New connection $peerInfo found", Toast.LENGTH_LONG).show()
        }
    }
}
