package org.perceivers25.warpinator.core.utils.messages

import androidx.compose.material3.SnackbarDuration
import androidx.compose.runtime.Composable
import androidx.compose.ui.res.stringResource
import org.perceivers25.warpinator.R
import org.perceivers25.warpinator.core.model.ui.UiMessage
import org.perceivers25.warpinator.core.model.ui.UiMessageState

class SucceededToWriteLog : UiMessage() {
    @Composable
    override fun getState(): UiMessageState {
        return UiMessageState(
            message = stringResource(R.string.dumped_log_file_to_selected_destination),
            duration = SnackbarDuration.Short,
        )
    }
}