package org.perceivers25.warpinator.core.data

/**
 * Represents the current connectivity status of the device for Warpinator usage.
 */
data class NetworkState(
    val isConnected: Boolean = false,
    val isHotspot: Boolean = false,
) {
    val isOnline: Boolean
        get() = isConnected || isHotspot
}
