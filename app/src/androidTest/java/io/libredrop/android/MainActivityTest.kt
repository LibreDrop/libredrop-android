package io.libredrop.android

import androidx.test.espresso.Espresso.onView
import androidx.test.espresso.IdlingRegistry
import androidx.test.espresso.assertion.ViewAssertions.matches
import androidx.test.espresso.matcher.ViewMatchers.isDisplayed
import androidx.test.espresso.matcher.ViewMatchers.withId
import androidx.test.rule.ActivityTestRule
import io.libredrop.android.testutils.ElapsedTimeIdlingResource
import org.junit.Rule
import org.junit.Test

class MainActivityTest {
    @Rule
    @JvmField
    var activityTestRule = ActivityTestRule(MainActivity::class.java)

    @Test
    fun appIsNotCrashing() {
        val idlingResource = ElapsedTimeIdlingResource(3_000)
        IdlingRegistry.getInstance().register(idlingResource)

        onView(withId(R.id.peers_list)).check(matches(isDisplayed()))

        IdlingRegistry.getInstance().unregister(idlingResource)
    }
}
