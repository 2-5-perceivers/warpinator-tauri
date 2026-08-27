package org.perceivers25.warpinator.feature.settings.components

import androidx.compose.foundation.layout.padding
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.semantics.heading
import androidx.compose.ui.semantics.semantics
import androidx.compose.ui.tooling.preview.Preview
import androidx.compose.ui.unit.dp

@Composable
fun SettingsCategoryLabel(title: String) {
    if (title.isNotEmpty()) {
        Text(
            text = title,
            style = MaterialTheme.typography.labelLarge,
            color = MaterialTheme.colorScheme.primary,
            modifier = Modifier
                .padding(horizontal = 10.dp)
                .padding(
                    top = 32.dp, bottom = 12.dp,
                )
                .semantics {
                    heading()
                },
        )
    }
}

@Preview(showBackground = true)
@Composable
fun SettingsCategoryLabelPreview() {
    SettingsCategoryLabel("Category Label")
}