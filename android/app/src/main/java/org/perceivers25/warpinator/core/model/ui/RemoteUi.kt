package org.perceivers25.warpinator.core.model.ui

import android.graphics.Bitmap
import org.perceivers25.warpinator.RemoteState

data class RemoteUi(
    var uuid: String = "",
    var ip: String = "",
    val displayName: String,
    var username: String,
    var hostname: String,
    var picture: Bitmap? = null,
    var pictureVersion: Byte = 0,
    var state: RemoteState = RemoteState.Connected,
    var messageSupport: Boolean = false,
    var isFavorite: Boolean = false,
    var unreadTransfers: Boolean = false,
    var unreadMessages: Boolean = false,
)
