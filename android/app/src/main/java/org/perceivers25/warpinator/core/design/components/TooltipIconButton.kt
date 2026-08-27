package org.perceivers25.warpinator.core.design.components

import androidx.compose.foundation.interaction.MutableInteractionSource
import androidx.compose.material3.Badge
import androidx.compose.material3.BadgedBox
import androidx.compose.material3.ExperimentalMaterial3Api
import androidx.compose.material3.Icon
import androidx.compose.material3.IconButton
import androidx.compose.material3.LocalContentColor
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.PlainTooltip
import androidx.compose.material3.Text
import androidx.compose.material3.TooltipAnchorPosition
import androidx.compose.material3.TooltipBox
import androidx.compose.material3.TooltipDefaults
import androidx.compose.material3.rememberTooltipState
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.graphics.vector.ImageVector

@OptIn(ExperimentalMaterial3Api::class)
@Composable
fun TooltipIconButton(
    description: String,
    modifier: Modifier = Modifier,
    interactionSource: MutableInteractionSource? = null,
    enabled: Boolean = true,
    onClick: () -> Unit,
    icon: ImageVector,
    tint: Color = if (enabled) LocalContentColor.current else LocalContentColor.current.copy(alpha = 0.38f),
    addBadge: Boolean = false,
) {
    TooltipBox(
        positionProvider = TooltipDefaults.rememberTooltipPositionProvider(TooltipAnchorPosition.Below),
        tooltip = { PlainTooltip { Text(description) } },
        state = rememberTooltipState(),
    ) {
        IconButton(
            onClick = onClick,
            enabled = enabled,
            modifier = modifier,
            interactionSource = interactionSource,
        ) {
            BadgedBox(
                badge = {
                    if (addBadge) {
                        Badge(containerColor = MaterialTheme.colorScheme.tertiary)
                    }
                },
            ) {
                Icon(
                    imageVector = icon,
                    contentDescription = description,
                    tint = tint,
                )
            }
        }
    }
}