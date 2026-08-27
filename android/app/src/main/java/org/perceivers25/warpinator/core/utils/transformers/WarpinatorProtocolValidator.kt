package org.perceivers25.warpinator.core.utils.transformers

import androidx.compose.foundation.ExperimentalFoundationApi
import androidx.compose.foundation.text.input.InputTransformation
import androidx.compose.foundation.text.input.TextFieldBuffer
import androidx.compose.foundation.text.input.delete

private const val WARPINATOR_SCHEME = "warpinator://"

/**
 * A validator utility class that can also transform input to be valid in a text box using InputTransformation
 * */
class ProtocolAddressInputValidator : InputTransformation {

    @OptIn(ExperimentalFoundationApi::class)
    override fun TextFieldBuffer.transformInput() {
        // In case of pasting the link from another device remove the protocol prefix

        if (this.changes.changeCount == 1) {
            val text = asCharSequence()
            if (text.startsWith(WARPINATOR_SCHEME)) {
                this.delete(0, WARPINATOR_SCHEME.length)
            }
        }

        val text = asCharSequence().toString()
        if (!text.all { it.isDigit() || it == '.' || it == ':' }) {
            revertAllChanges()
        }
        if (!isValidIp(text)) {
            revertAllChanges()
        }
    }

    companion object {
        val scheme: String
            get() = WARPINATOR_SCHEME

        fun isValidIp(text: String, allowPartial: Boolean = true): Boolean {
            // Prevent starting with a dot or colon
            if (text.startsWith(".") || text.startsWith(":")) return false

            // Handle Port separation (IP:PORT)
            val parts = text.split(":")
            if (parts.size > 2) return false // Only one colon allowed

            val ipPart = parts[0]
            val portPart = if (parts.size > 1) parts[1] else null

            val segments = ipPart.split(".")
            if (segments.size > 4) return false // Max 4 segments (x.x.x.x)

            for (segment in segments) {
                if (segment.isEmpty()) continue

                if (segment.length > 3) return false

                val value = segment.toIntOrNull() ?: return false
                if (value > 255) return false
            }

            // Validate Port Part
            if (portPart != null) {
                if (portPart.isEmpty() && allowPartial) return true
                val port = portPart.toIntOrNull() ?: return false
                if (port > 65535) return false
            }

            return allowPartial || (segments.size == 4 && portPart != null)
        }

        fun getFullAddressUrl(address: String): String {
            return "$WARPINATOR_SCHEME$address"
        }
    }

}