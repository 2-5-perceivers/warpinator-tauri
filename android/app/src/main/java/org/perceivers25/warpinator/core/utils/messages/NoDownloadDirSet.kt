package org.perceivers25.warpinator.core.utils.messages

import androidx.compose.material3.SnackbarDuration
import androidx.compose.runtime.Composable
import org.perceivers25.warpinator.core.model.ui.UiMessage
import org.perceivers25.warpinator.core.model.ui.UiMessageState

class NoDownloadDirSet : UiMessage() {
    @Composable
    override fun getState(): UiMessageState {
        return UiMessageState(
            message = "No download directory set",
            duration = SnackbarDuration.Long,
        )
    }
}