"use client"

import { useState, useEffect } from "react"
import "../styles/BlockExplorer.css"

const BlockExplorer = () => {
  const [blocks, setBlocks] = useState([])
  const [loading, setLoading] = useState(true)
  const [selectedBlock, setSelectedBlock] = useState(null)

  useEffect(() => {
    fetchBlocks()
  }, [])

  const fetchBlocks = async () => {
    try {
      const response = await fetch("http://localhost:3001/api/blockchain/blocks")
      if (response.ok) {
        const data = await response.json()
        setBlocks(data)
      }
    } catch (err) {
      console.error("Failed to fetch blocks", err)
    } finally {
      setLoading(false)
    }
  }

  if (loading) return <div className="loading">Loading...</div>

  return (
    <div className="block-explorer">
      <div className="explorer-container">
        <h1>Blockchain Explorer</h1>

        <div className="blocks-grid">
          {blocks.map((block) => (
            <div key={block.hash} className="block-card" onClick={() => setSelectedBlock(block)}>
              <h3>Block #{block.index}</h3>
              <p className="hash">Hash: {block.hash.substring(0, 24)}...</p>
              <p className="timestamp">{new Date(block.timestamp).toLocaleString()}</p>
              <p className="tx-count">Transactions: {block.transactions.length}</p>
            </div>
          ))}
        </div>

        {selectedBlock && (
          <div className="block-detail">
            <h2>Block #{selectedBlock.index} Details</h2>
            <div className="detail-content">
              <p>
                <strong>Hash:</strong> <code>{selectedBlock.hash}</code>
              </p>
              <p>
                <strong>Previous Hash:</strong> <code>{selectedBlock.previousHash}</code>
              </p>
              <p>
                <strong>Nonce:</strong> {selectedBlock.nonce}
              </p>
              <p>
                <strong>Timestamp:</strong> {new Date(selectedBlock.timestamp).toLocaleString()}
              </p>
              <p>
                <strong>Merkle Root:</strong> <code>{selectedBlock.merkleRoot}</code>
              </p>

              <h3>Transactions</h3>
              {selectedBlock.transactions.map((tx, idx) => (
                <div key={idx} className="tx-detail">
                  <p>
                    <strong>From:</strong> {tx.senderWalletId}
                  </p>
                  <p>
                    <strong>To:</strong> {tx.recipientWalletId}
                  </p>
                  <p>
                    <strong>Amount:</strong> {tx.amount} CWC
                  </p>
                  <p>
                    <strong>Signature:</strong> <code>{tx.signature.substring(0, 32)}...</code>
                  </p>
                </div>
              ))}
            </div>
            <button onClick={() => setSelectedBlock(null)} className="close-btn">
              Close
            </button>
          </div>
        )}
      </div>
    </div>
  )
}

export default BlockExplorer
