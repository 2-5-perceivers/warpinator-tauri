package org.perceivers25.warpinator.core.utils.transformers

import androidx.compose.foundation.text.input.OutputTransformation
import androidx.compose.foundation.text.input.TextFieldBuffer
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.text.SpanStyle
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.unit.sp

class IPAddressTransformer(
    private val punctuationColor: Color,
) : OutputTransformation {
    val spanStyle = SpanStyle(
        color = punctuationColor, fontWeight = FontWeight.Bold,
        // Increased spacing makes the dots breathe more
        letterSpacing = 8.sp,
    )

    override fun TextFieldBuffer.transformOutput() {
        for ((i, c) in asCharSequence().withIndex()) {
            if (c == '.' || c == ':') {
                // Visual replacement
                if (c == '.') {
                    replace(i, i + 1, "•")
                }

                // Styling
                addStyle(
                    spanStyle, i, i + 1,
                )
            }
        }
    }
}