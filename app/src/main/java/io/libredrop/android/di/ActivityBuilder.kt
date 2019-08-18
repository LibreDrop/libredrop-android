package io.libredrop.android.di

import dagger.Module
import dagger.android.ContributesAndroidInjector
import io.libredrop.android.activity.PeersActivity
import io.libredrop.android.activity.PeersDIModule

@Module
interface ActivityBuilder {

    @ContributesAndroidInjector(modules = [PeersDIModule::class])
    fun bindMainActivity(): PeersActivity
}
