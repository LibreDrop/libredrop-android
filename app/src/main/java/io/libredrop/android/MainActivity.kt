package io.libredrop.android

import android.app.Activity
import android.os.Bundle

class MainActivity : Activity() {

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        setContentView(R.layout.activity_main)

        startDiscovery()
    }

    companion object {

        @JvmStatic
        external fun startDiscovery()

        // Used to load the 'native-lib' library on application startup.
        init {
            System.loadLibrary("rust")
        }
    }
}
