package io.libredrop.android

import android.view.LayoutInflater
import android.view.View
import android.view.ViewGroup
import android.widget.TextView
import androidx.recyclerview.widget.RecyclerView
import io.libredrop.network.PeerInfo
import kotlin.properties.Delegates

class PeersAdapter : RecyclerView.Adapter<PeersAdapter.PeersViewHolder>() {

    var peers: List<PeerInfo> by Delegates.observable(emptyList()) { _, _, new ->
        notifyDataSetChanged()
    }

    override fun onCreateViewHolder(parent: ViewGroup, viewType: Int): PeersAdapter.PeersViewHolder {
        val view = LayoutInflater.from(parent.context).inflate(android.R.layout.simple_list_item_2, parent, false)
        return PeersViewHolder(view)
    }

    override fun getItemCount(): Int = peers.size

    override fun onBindViewHolder(holder: PeersAdapter.PeersViewHolder, position: Int) {
        val peerInfo = peers[position]

        with(holder.itemView) {
            findViewById<TextView>(android.R.id.text1).text = "#${peerInfo.id} ${peerInfo.name}"
            findViewById<TextView>(android.R.id.text2).text = peerInfo.ip
        }
    }

    class PeersViewHolder(view: View) : RecyclerView.ViewHolder(view)
}
