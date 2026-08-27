package org.perceivers25.warpinator.feature.settings.messages

import androidx.compose.material3.SnackbarDuration
import androidx.compose.runtime.Composable
import androidx.compose.ui.res.stringResource
import org.perceivers25.warpinator.R
import org.perceivers25.warpinator.core.model.ui.UiMessage
import org.perceivers25.warpinator.core.model.ui.UiMessageState

class NeedsRestartMessage : UiMessage() {
    @Composable
    override fun getState(): UiMessageState {
        return UiMessageState(
            message = stringResource(R.string.requires_restart_warning),
            duration = SnackbarDuration.Long,
        )
    }
}