package ch.micmine.qutelsp

import com.intellij.lang.Language
import com.intellij.openapi.application.PreloadingActivity
import org.wso2.lsp4intellij.IntellijLanguageClient
import org.wso2.lsp4intellij.client.languageserver.serverdefinition.RawCommandServerDefinition

class QuteLspPreload: PreloadingActivity() {
    override suspend fun execute() {
        println(Language.getRegisteredLanguages())
        IntellijLanguageClient.addServerDefinition(RawCommandServerDefinition("html", arrayOf("qute-lsp")))
    }
}