package org.perceivers25.warpinator.core.model.preferences

import kotlinx.serialization.Serializable
import kotlinx.serialization.json.Json

@Serializable
data class RecentRemote(val host: String, val hostname: String)

fun List<RecentRemote>.toJson(): String {
    return Json.encodeToString(this)
}

fun recentRemotesFromJson(json: String): List<RecentRemote> {
    return Json.decodeFromString(json)
}