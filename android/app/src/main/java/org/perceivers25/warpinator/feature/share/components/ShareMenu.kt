package org.perceivers25.warpinator.feature.share.components

import androidx.compose.foundation.interaction.MutableInteractionSource
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.aspectRatio
import androidx.compose.foundation.layout.height
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.layout.wrapContentSize
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.rounded.AddLink
import androidx.compose.material.icons.rounded.MoreVert
import androidx.compose.material3.ExperimentalMaterial3Api
import androidx.compose.material3.ExperimentalMaterial3ExpressiveApi
import androidx.compose.material3.Icon
import androidx.compose.material3.IconButton
import androidx.compose.material3.IconButtonDefaults
import androidx.compose.material3.ListItemDefaults
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Scaffold
import androidx.compose.runtime.Composable
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.saveable.rememberSaveable
import androidx.compose.runtime.setValue
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.res.stringResource
import androidx.compose.ui.tooling.preview.Preview
import androidx.compose.ui.unit.Dp
import androidx.compose.ui.unit.DpOffset
import androidx.compose.ui.unit.dp
import org.perceivers25.warpinator.R
import org.perceivers25.warpinator.core.design.components.MenuAction
import org.perceivers25.warpinator.core.design.components.MenuGroup
import org.perceivers25.warpinator.core.design.components.MenuGroupsPopup
import org.perceivers25.warpinator.core.design.shapes.segmentedHorizontalDynamicShapes
import org.perceivers25.warpinator.core.design.shapes.toIconButtonShapes

@OptIn(ExperimentalMaterial3ExpressiveApi::class, ExperimentalMaterial3Api::class)
@Composable
fun ShareMenu(
    initiallyExpanded: Boolean = false,
    onManualConnectionClick: () -> Unit,
    size: Dp = 48.dp,
) {
    var menuOpen by rememberSaveable { mutableStateOf(initiallyExpanded) }
    val groupInteractionSource = remember { MutableInteractionSource() }

    val menuGroups = listOf(
        MenuGroup(
            listOf(
                MenuAction(
                    stringResource(R.string.manual_connection_label),
                    trailingIcon = Icons.Rounded.AddLink,
                    onClick = onManualConnectionClick,
                ),
            ),
        ),
    )

    Box(
        modifier = Modifier.wrapContentSize(Alignment.TopEnd),
    ) {
        IconButton(
            onClick = {
                menuOpen = true
            },
            modifier = Modifier
                .padding(start = ListItemDefaults.SegmentedGap)
                .height(size)
                .aspectRatio(1f),
            shapes = ListItemDefaults.segmentedHorizontalDynamicShapes(
                index = 1,
                count = 2,
            ).toIconButtonShapes(),
            colors = IconButtonDefaults.iconButtonColors(
                containerColor = MaterialTheme.colorScheme.secondaryContainer,
                contentColor = MaterialTheme.colorScheme.onSecondaryContainer,
            ),
        ) {
            Icon(
                Icons.Rounded.MoreVert,
                contentDescription = stringResource(R.string.open_menu_label),
            )
        }
        MenuGroupsPopup(
            menuOpen,
            menuGroups,
            groupInteractionSource,
            onDismiss = { menuOpen = false },
            offset = DpOffset(y = 2.dp, x = 0.dp),
        )
    }
}

@Preview(showBackground = true)
@Composable
fun HomeMenuPreview() {
    Scaffold { paddingValues ->
        Box(modifier = Modifier.padding(paddingValues)) {
            ShareMenu(true, onManualConnectionClick = {})
        }
    }
}