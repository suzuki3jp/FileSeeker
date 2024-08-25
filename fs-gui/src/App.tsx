import { ThemeProvider, createTheme, Grid, CssBaseline } from "@mui/material";
import { FilePathInput } from "./components";

const greyishDarkTheme = createTheme({
  palette: {
    mode: "dark",
    background: {
      default: "#303030", // より明るいグレー
      paper: "#424242", // 少し明るいグレー（カード、ダイアログなどの背景）
    },
    text: {
      primary: "#ffffff", // 白テキスト
      secondary: "#b0b0b0", // 薄いグレーのテキスト
    },
    primary: {
      main: "#90caf9", // 明るい青（アクセントカラー）
    },
    secondary: {
      main: "#f48fb1", // ピンク（セカンダリーアクセントカラー）
    },
  },
});

function App() {
  return (
    <ThemeProvider theme={greyishDarkTheme}>
      <CssBaseline />
      <Grid container sx={{ padding: "1% 1% 1% 1%" }}>
        <FilePathInput />
      </Grid>
    </ThemeProvider>
  );
}

export default App;
