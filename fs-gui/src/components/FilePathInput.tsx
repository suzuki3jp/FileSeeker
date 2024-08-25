import { useState, type FC } from "react";
import { Grid, TextField, Button } from "@mui/material";
import { open } from "@tauri-apps/api/dialog";

export const FilePathInput: FC = () => {
  const [filePath, setFilePath] = useState<string>();

  const openFilePickerHandler = async () => {
    const result = await open({ directory: true, multiple: false });
    if (typeof result !== "string") return;
    setFilePath(result);
  };

  return (
    <Grid item xs={8}>
      <Grid container spacing={2} alignItems="center">
        <Grid item xs={10}>
          <TextField size="small" fullWidth value={filePath}></TextField>
        </Grid>
        <Grid item xs={2}>
          <Button variant="contained" onClick={openFilePickerHandler}>
            Browse
          </Button>
        </Grid>
      </Grid>
    </Grid>
  );
};
