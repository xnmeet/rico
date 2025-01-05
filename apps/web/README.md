# Rico Playground

Rico Playground is a modern web application that provides a seamless interface for converting between Thrift IDL and JSON formats. It features a real-time conversion experience with a beautiful, responsive UI.

## Features

- 🔄 Bi-directional conversion between Thrift IDL and JSON
- ⚡️ Real-time conversion with debouncing
- 🎨 Modern UI with glassmorphism design
- 🌓 Light/Dark theme support
- 📝 Rich code editor with syntax highlighting
- 📊 Performance metrics display
- 💫 Smooth animations and transitions
- 🎯 Copy to clipboard functionality

## Tech Stack

- **Framework**: React 19
- **Build Tool**: Rsbuild
- **UI Components**:
  - CodeMirror 6 for code editing
  - Lucide React for icons
  - Framer Motion for animations
  - Tailwind CSS for styling
- **Core Features**:
  - WASM-based Thrift parser
  - Real-time conversion
  - Theme management
  - Error handling

## Development

### Prerequisites

- Node.js 18+
- pnpm 8+

### Getting Started

1. Install dependencies:

```bash
pnpm install
```

2. Start the development server:

```bash
pnpm dev
```

3. Build for production:

```bash
pnpm build
```

4. Preview production build:

```bash
pnpm preview
```

## Project Structure

```
src/
├── components/        # React components
│   ├── converter/    # Conversion related components
│   ├── editor/      # Code editor components
│   ├── layout/      # Layout components
│   ├── logo/        # Logo components
│   ├── status/      # Status indicators
│   ├── theme/       # Theme management
│   └── ui/          # Shared UI components
├── lib/             # Utilities and services
│   ├── constants/   # Constants and types
│   ├── examples/    # Example code snippets
│   ├── parser/      # Parser service
│   ├── rico/        # WASM integration
│   └── utils/       # Helper functions
└── App.tsx          # Root component
```

## Contributing

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## License

This project is licensed under the MIT License.
