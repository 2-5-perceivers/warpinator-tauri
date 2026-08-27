package org.perceivers25.warpinator.core.utils

import android.content.Context
import android.net.Uri
import android.provider.DocumentsContract

fun checkWillOverwrite(context: Context, entries: List<String>, pathUri: Uri): Boolean {
    if (entries.isEmpty()) return false

    val treeDocId = DocumentsContract.getTreeDocumentId(pathUri)
    val childrenUri = DocumentsContract.buildChildDocumentsUriUsingTree(pathUri, treeDocId)

    context.contentResolver.query(
        childrenUri,
        arrayOf(DocumentsContract.Document.COLUMN_DISPLAY_NAME),
        null, null, null,
    )?.use { cursor ->
        val nameIndex = cursor.getColumnIndexOrThrow(DocumentsContract.Document.COLUMN_DISPLAY_NAME)
        while (cursor.moveToNext()) {
            if (cursor.getString(nameIndex) in entries) return true
        }
    }

    return false
}