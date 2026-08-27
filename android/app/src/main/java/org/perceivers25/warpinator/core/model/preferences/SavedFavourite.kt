package org.perceivers25.warpinator.core.model.preferences

import kotlinx.serialization.Serializable
import kotlinx.serialization.json.Json

@Serializable
data class SavedFavourite(val uuid: String)

fun Set<SavedFavourite>.toJson(): String {
    return Json.encodeToString(this)
}

fun savedFavouritesFromJson(json: String): Set<SavedFavourite> {
    return Json.decodeFromString(json)
}