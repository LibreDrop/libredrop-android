package io.libredrop.android

import android.os.Bundle
import androidx.fragment.app.FragmentActivity
import androidx.lifecycle.Observer
import androidx.lifecycle.ViewModelProviders
import androidx.recyclerview.widget.LinearLayoutManager
import io.libredrop.network.PeerInfo
import kotlinx.android.synthetic.main.activity_main.*

class MainActivity : FragmentActivity() {

    private lateinit var peersModel: PeersViewModel

    private val adapter = PeersAdapter()

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        setContentView(R.layout.activity_main)

        peers_list.layoutManager = LinearLayoutManager(this)
        peers_list.adapter = adapter

        peersModel = ViewModelProviders.of(this).get(PeersViewModel::class.java)
        peersModel.peers.observe(this, Observer<List<PeerInfo>> { peers ->
            adapter.peers = peers
        })
    }
}
