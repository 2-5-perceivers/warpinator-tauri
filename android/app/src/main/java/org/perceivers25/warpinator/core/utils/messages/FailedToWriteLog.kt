package org.perceivers25.warpinator.core.utils.messages

import androidx.compose.material3.SnackbarDuration
import androidx.compose.runtime.Composable
import androidx.compose.ui.res.stringResource
import org.perceivers25.warpinator.R
import org.perceivers25.warpinator.core.model.ui.UiMessage
import org.perceivers25.warpinator.core.model.ui.UiMessageState

class FailedToWriteLog(val exception: Exception) : UiMessage() {
    @Composable
    override fun getState(): UiMessageState {
        return UiMessageState(
            message = stringResource(
                R.string.could_not_save_log_to_file_message,
                exception.message ?: stringResource(R.string.unknown_error),
            ),
            duration = SnackbarDuration.Long,
        )
    }
}