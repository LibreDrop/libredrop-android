package io.libredrop.android

import android.os.Bundle
import androidx.fragment.app.FragmentActivity
import androidx.lifecycle.*
import dagger.android.AndroidInjection
import dagger.android.AndroidInjector
import dagger.android.DispatchingAndroidInjector
import dagger.android.HasAndroidInjector
import javax.inject.Inject

abstract class BaseActivity : FragmentActivity(), HasAndroidInjector {
    @Inject
    lateinit var androidInjector: DispatchingAndroidInjector<Any>

    @Inject
    lateinit var viewModelFactory: ViewModelProvider.Factory

    override fun onCreate(savedInstanceState: Bundle?) {
        AndroidInjection.inject(this)
        super.onCreate(savedInstanceState)
    }

    override fun androidInjector(): AndroidInjector<Any> {
        return androidInjector
    }

    protected inline fun <reified T : ViewModel> FragmentActivity.getViewModel(): T {
        return ViewModelProviders.of(this, viewModelFactory)[T::class.java]
    }

    protected inline fun <reified T : ViewModel> withViewModel(body: T.() -> Unit): T {
        val vm: T = getViewModel()
        vm.body()
        return vm
    }

    protected fun <T : Any?, L : LiveData<T>> LifecycleOwner.observe(liveData: L, body: (T) -> Unit) {
        liveData.observe(this, Observer<T>(body))
    }
}
