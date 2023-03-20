// components/CodeEditor.tsx

import React from 'react';
import AceEditor from 'react-ace';

import 'ace-builds/src-noconflict/mode-rust';
import 'ace-builds/src-noconflict/theme-one_dark';

interface CodeEditorProps {
    code: string;
    onChange: (newCode: string) => void;
}

const CodeEditor: React.FC<CodeEditorProps> = ({ code, onChange }) => {
    return (
        <div className="w-screen h-screen overflow-hidden relative">
            <AceEditor
                mode="rust"
                theme="one_dark"
                value={code}
                onChange={onChange}
                fontSize={14}
                width={'100%'}
                height={'100%'}
                showPrintMargin={true}
                showGutter={true}
                highlightActiveLine={true}
                setOptions={{
                    enableBasicAutocompletion: true,
                    enableSnippets: true,
                    showLineNumbers: true,
                    tabSize: 4,
                }}
                className="w-screen h-screen"
            />
        </div>
    );
};

export default CodeEditor;
