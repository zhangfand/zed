'use client';

import React from 'react';
import { Comment as CommentType } from '../../data/conversations';
import { Avatar } from '@radix-ui/react-avatar';
import { formatDistanceToNow } from 'date-fns';
import * as Popover from '@radix-ui/react-popover';
import * as Tooltip from '@radix-ui/react-tooltip';
import { DotsHorizontalIcon, Link2Icon } from '@radix-ui/react-icons';
import styled from '@emotion/styled';
import ReactMarkdown from 'react-markdown';
import gfm from 'remark-gfm';
import { Prism as SyntaxHighlighter } from 'react-syntax-highlighter';
import rust from 'react-syntax-highlighter/dist/cjs/languages/prism/rust';
import rangeParser from 'parse-numeric-range';
import { oneDark } from 'react-syntax-highlighter/dist/cjs/styles/prism';


const CommentContainer = styled.div`
    background-color: #282c34;
    padding: 16px;
    border-radius: 4px;
    box-shadow: 0 1px 3px rgba(0, 0, 0, 0.12), 0 1px 2px rgba(0, 0, 0, 0.24);
    margin-bottom: 8px;

    &:hover {
        .menu {
            display: flex;
        }
    }
`;

const CommentMetadata = styled.div`
    display: flex;
    gap: 8px;
    align-items: center;

    .author {
        font-size: 0.875rem;
        font-weight: 500;
        color: #abb2bf;
    }

    .timestamp {
        font-size: 0.75rem;
        color: #545862;
    }
`;

const CommentMessage = styled.p`
    font-size: 0.875rem;
    line-height: 1.5;
    color: #abb2bf;
`;

const CommentTimestamp = styled.span`
    font-size: 0.75rem;
    color: #545862;
`;

const Button = styled.button`
    background-color: transparent;
    border: none;
    cursor: pointer;
    color: #545862;

    &:hover {
        color: #abb2bf;
    }
`;

interface MarkdownProps {
    content: string;
}

const Markdown: React.FC<MarkdownProps> = ({ content }) => {
  const syntaxTheme = oneDark;

  const MarkdownComponents = {
    code({ node, inline, className, ...props }) {
      const hasLang = /language-(\w+)/.exec(className || '');
      const hasMeta = node?.data?.meta;

      const applyHighlights = (applyHighlights: number) => {
        if (hasMeta) {
          const RE = /{([\d,-]+)}/;
          const metadata = node.data.meta?.replace(/\s/g, '');
          const strlineNumbers = RE?.test(metadata)
            ? RE?.exec(metadata)[1]
            : '0';
          const highlightLines = rangeParser(strlineNumbers);
          const highlight = highlightLines;
          const data: string = highlight.includes(applyHighlights)
            ? 'highlight'
            : null;
          return { data };
        } else {
          return {};
        }
      };

      return hasLang ? (
        <SyntaxHighlighter
          style={syntaxTheme}
          language={hasLang[1]}
          PreTag="div"
          className="codeStyle"
          showLineNumbers={true}
          wrapLines={hasMeta}
          useInlineStyles={true}
          lineProps={applyHighlights}
        >
          {props.children}
        </SyntaxHighlighter>
      ) : (
        <code className={className} {...props} />
      );
    },
  };

  return (
    <ReactMarkdown
      components={MarkdownComponents}
      remarkPlugins={[gfm]}
    >
      {content}
    </ReactMarkdown>
  );
};


interface CommentProps {
    comment: CommentType;
    onDelete: (id: number) => void;
}

const Comment: React.FC<CommentProps> = ({ comment, onDelete }) => {
    const authorColor = `hsl(${comment.author.length * 15}, 70%, 50%)`;

    const message = <Markdown content={comment.message} />

    return (
        <CommentContainer>
            <div className="flex justify-between">
                <div className="flex justify-between items-center mb-2">
                    <div className="flex items-center">
                        <Avatar
                            className="mr-2"
                            style={{ backgroundColor: authorColor }}
                        >
                            {comment.author.charAt(0)}
                        </Avatar>
                        <CommentMetadata>
                            <div className="author">{comment.author}</div>
                            <div className="timestamp">
                                {formatDistanceToNow(new Date(comment.timestamp))} ago
                            </div>
                        </CommentMetadata>
                    </div>
                </div>

                <menu className="menu hidden space-x-4">
                    <Tooltip.Provider>
                        <Tooltip.Root>
                            <Tooltip.Trigger>
                                <Button>
                                    <Link2Icon />
                                </Button>
                            </Tooltip.Trigger>
                            <Tooltip.Portal>
                                <Tooltip.Content>
                                    <div className='bg-white border border-black/5 py-1 px-2 text-xs rounded-lg shadow'>Copy Link</div>
                                </Tooltip.Content>
                            </Tooltip.Portal>
                        </Tooltip.Root>
                        <Popover.Root>
                            <Popover.Trigger asChild>
                                <Button>
                                    <DotsHorizontalIcon />
                                </Button>
                            </Popover.Trigger>
                            <Popover.Content className="bg-white shadow-md rounded-md p-2">
                                <button className="block text-xs w-full text-left text-gray-700 hover:bg-gray-100 py-1 px-2">
                                    Reply to Comment
                                </button>
                                <button className="block text-xs w-full text-left text-gray-700 hover:bg-gray-100 py-1 px-2">
                                    Edit Comment
                                </button>
                                <Button
                                    onClick={() => onDelete(comment.id)}
                                    className="block text-xs w-full text-left text-red-500 hover:text-red-700 hover:bg-gray-100 py-1 px-2"
                                >
                                    Delete Comment
                                </Button>
                            </Popover.Content>
                        </Popover.Root>
                    </Tooltip.Provider>
                </menu>
            </div>
            <CommentMessage>{message}</CommentMessage>
            <CommentTimestamp>{new Date(comment.timestamp).toLocaleString()}</CommentTimestamp>
        </CommentContainer>
    );
};

export default Comment;
