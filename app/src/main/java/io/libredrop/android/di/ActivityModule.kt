package io.libredrop.android.di

import androidx.lifecycle.ViewModelProvider
import dagger.Binds
import dagger.Module
import io.libredrop.android.ViewModelFactory

@Module(includes = [ActivityBuilder::class])
abstract class ActivityModule {
    @Binds
    abstract fun bindViewModelFactory(factory: ViewModelFactory): ViewModelProvider.Factory
}
