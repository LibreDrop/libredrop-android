package io.libredrop.android.activity

import android.view.LayoutInflater
import android.view.View
import android.view.ViewGroup
import androidx.recyclerview.widget.RecyclerView
import io.libredrop.android.R
import io.libredrop.network.PeerInfo
import kotlinx.android.synthetic.main.peer_item.view.*
import kotlin.properties.Delegates

class PeersAdapter(
    val onSendClick: (PeerInfo) -> Unit
) : RecyclerView.Adapter<PeersAdapter.PeersViewHolder>() {

    var peers: List<PeerInfo> by Delegates.observable(emptyList()) { _, _, _ ->
        notifyDataSetChanged()
    }

    override fun onCreateViewHolder(parent: ViewGroup, viewType: Int): PeersViewHolder {
        val view = LayoutInflater.from(parent.context).inflate(R.layout.peer_item, parent, false)
        return PeersViewHolder(view)
    }

    override fun getItemCount(): Int = peers.size

    override fun onBindViewHolder(holder: PeersViewHolder, position: Int) {
        val peerInfo = peers[position]

        with(holder.itemView) {
            peer_name.text = "#${peerInfo.id} ${peerInfo.name}"
            peer_info.text = peerInfo.ip
            peer_send.setOnClickListener {
                onSendClick(peerInfo)
            }
        }
    }

    class PeersViewHolder(view: View) : RecyclerView.ViewHolder(view)
}
