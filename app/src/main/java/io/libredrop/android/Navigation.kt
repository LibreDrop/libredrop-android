package io.libredrop.android

import android.app.Activity
import android.content.Intent
import android.net.Uri
import com.github.florent37.inlineactivityresult.kotlin.coroutines.startForResult
import javax.inject.Inject

class Navigation @Inject constructor(val activity: BaseActivity) {
    /**
     * @return null means user did not select any file
     */
    suspend fun openFileSelection(): Uri? {
        val intent = Intent(Intent.ACTION_GET_CONTENT).apply {
            type = "*/*"
        }
        val result = activity.startForResult(intent)

        if (result.resultCode != Activity.RESULT_OK) return null

        return result.data?.data
    }
}
