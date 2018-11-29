package io.libredrop.android

import android.app.Activity
import android.os.Bundle
import android.widget.Toast
import io.libredrop.network.Network

class MainActivity : Activity() {

    private val network = Network(::onNewConnectionFound)

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        setContentView(R.layout.activity_main)
    }

    override fun onStart() {
        super.onStart()

        network.startDiscovery()
    }

    override fun onStop() {
        super.onStop()

        network.stopDiscovery()
    }

    private fun onNewConnectionFound(name: String) {
        Toast.makeText(this, "New connection $name found", Toast.LENGTH_LONG).show()
    }
}
