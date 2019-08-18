package io.libredrop.android

import dagger.android.AndroidInjector
import dagger.android.DaggerApplication
import io.libredrop.android.di.DaggerApplicationComponent

class Application : DaggerApplication() {

    override fun applicationInjector(): AndroidInjector<Application> {
        return DaggerApplicationComponent.factory()
            .build(this)
    }
}
