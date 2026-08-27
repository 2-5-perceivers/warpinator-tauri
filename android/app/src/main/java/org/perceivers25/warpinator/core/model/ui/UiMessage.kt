package org.perceivers25.warpinator.core.model.ui

import androidx.compose.material3.SnackbarDuration
import androidx.compose.runtime.Composable

data class UiMessageState(
    val message: String,
    val duration: SnackbarDuration = SnackbarDuration.Short,
    val type: MessageType = MessageType.Snackbar,
)

enum class MessageType {
    Snackbar, Toast
}

abstract class UiMessage {
    @Composable
    abstract fun getState(): UiMessageState
    open val id: Any? = null
}
