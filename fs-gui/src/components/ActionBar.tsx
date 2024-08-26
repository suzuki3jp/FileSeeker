import type { FC } from "react";
import { useState } from "react";
import { Button, Grid } from "@mui/material";
import { invoke } from "@tauri-apps/api";

import { FilePathInput } from "./FilePathInput";

export const ActionBar: FC = () => {
  const [filePath, setFilePath] = useState("");

  const startButtonHandler = async () => {
    const path = filePath;
    await invoke("analyze", { path });
  };

  return (
    <>
      <Grid container spacing={4} alignItems="center">
        <FilePathInput filePath={filePath} setFilePath={setFilePath} />
        <Grid item xs={4}>
          <Button
            variant="contained"
            color="warning"
            onClick={startButtonHandler}
          >
            Start
          </Button>
        </Grid>
      </Grid>
    </>
  );
};
