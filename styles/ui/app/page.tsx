'use client';

// pages/index.tsx

/** @jsxImportSource @emotion/react */
import styled from '@emotion/styled'
import React, { useState } from 'react';
import Head from 'next/head';
import CodeEditor from './(editor)/CodeEditor';
import Conversation from './(conversation)/Conversation';
import { code as rustCode } from '../data/code'
import clsx from 'clsx';

const Home: React.FC = () => {
    const [code, setCode] = useState(rustCode);

    const handleCodeChange = (newCode: string) => {
        setCode(newCode);
    };

    const Main = styled.div`
            background-color: #282c34;
            color: #abb2bf;
        `;

    const Panel = styled.div`
            background-color: #1e2127;
            border-color: #181a1f;
        `;

    const PanelHeader = styled.div`
            background-color: #21252b;
            border-color: #181a1f;
        `;

    const Button = styled.div`
            background-color: #3b4048;
            color: #abb2bf;
            padding: 5px 10px;
            border-radius: 4px;
            cursor: pointer;

            &:hover {
                background-color: #4c5666;
            }
        `;

    return (
        <div className="min-h-screen bg-gray-100">
            <Head>
                <title>Code Editor</title>
                <meta name="description" content="A simple code editor using Next.js, React, TypeScript, and Tailwind CSS" />
                <link rel="icon" href="/favicon.ico" />
            </Head>

            <Main className={clsx('w-screen h-screen')}>
                <div className="w-full h-full bg-white">
                    <CodeEditor code={code} onChange={handleCodeChange} />
                </div>
                {/* Floating panel */}
                <Panel className={clsx('fixed top-10 right-10 bottom-10 rounded w-[460px] bg-white shadow-2xl overflow-y-auto')}>
                    {/* Panel header */}
                    <PanelHeader className="p-4 border-b flex justify-between items-center">
                        <div>
                            <h3 className="text-lg font-semibold">Issue #123 - Title</h3>
                            <p className="text-sm text-gray-500">Updated 5 minutes ago</p>
                        </div>
                        <Button className="bg-gray-200 p-2 rounded" onClick={() => {/* Close button logic */ }}>Close</Button>
                    </PanelHeader>

                    {/* Comment thread */}
                    <Conversation />
                </Panel>
            </Main>
        </div>
    );
};

export default Home;
