package io.libredrop.android

import androidx.annotation.CallSuper
import androidx.lifecycle.ViewModel
import kotlinx.coroutines.CoroutineScope
import kotlinx.coroutines.MainScope
import kotlinx.coroutines.cancel

abstract class BaseViewModel : ViewModel(), CoroutineScope by MainScope() {
    @CallSuper
    override fun onCleared() {
        cancel()
    }
}
