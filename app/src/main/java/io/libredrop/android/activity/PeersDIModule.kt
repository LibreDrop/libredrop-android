package io.libredrop.android.activity

import androidx.lifecycle.ViewModel
import dagger.Binds
import dagger.Module
import dagger.multibindings.IntoMap
import io.libredrop.android.BaseActivity
import io.libredrop.android.di.ViewModelKey

@Module
abstract class PeersDIModule {
    @Binds
    @IntoMap
    @ViewModelKey(PeersViewModel::class)
    abstract fun peersViewModel(viewModel: PeersViewModel): ViewModel

    @Binds
    abstract fun baseActivity(peersActivity: PeersActivity): BaseActivity
}
