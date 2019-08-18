package io.libredrop.android.activity

import android.os.Bundle
import androidx.recyclerview.widget.LinearLayoutManager
import io.libredrop.android.BaseActivity
import io.libredrop.android.R
import kotlinx.android.synthetic.main.activity_main.*

class PeersActivity : BaseActivity() {

    private lateinit var peersModel: PeersViewModel

    private lateinit var adapter: PeersAdapter

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        setContentView(R.layout.activity_main)

        peersModel = getViewModel()

        observe(peersModel.peers) {
            adapter.peers = it
        }

        adapter = PeersAdapter(peersModel::onSendClick)

        peers_list.layoutManager = LinearLayoutManager(this)
        peers_list.adapter = adapter
    }
}
