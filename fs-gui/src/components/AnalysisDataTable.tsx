import type { Dispatch, SetStateAction } from "react";
import {
  TableContainer,
  Table,
  TableHead,
  TableBody,
  Paper,
  TableRow,
  TableCell,
  Grid,
} from "@mui/material";

import { InvokeAnalyzeResult } from "../typings";

export const AnalysisDataTable = (props: AnalysisDataTableProps) => {
  const { results } = props;

  return (
    <Grid item xs={12}>
      <TableContainer component={Paper} sx={{ marginTop: "2%" }}>
        <Table size="small">
          <TableHead>
            <TableRow>
              <TableCell>Path</TableCell>
              <TableCell>Extension</TableCell>
              <TableCell>Charactors</TableCell>
              <TableCell>Lines</TableCell>
              <TableCell>Size (B)</TableCell>
            </TableRow>
          </TableHead>
          <TableBody>
            {results.map((data) => {
              const displayData: DisplayAnalysisData = {
                path: data.path_parts.join("/"),
                extension: data.extension,
                char: data.char,
                line: data.line,
                size: data.size,
              };
              const { path, extension, char, line, size } = displayData;

              return (
                <TableRow key={path}>
                  <TableCell component={"th"} scope="cell">
                    {path}
                  </TableCell>
                  <TableCell>{extension}</TableCell>
                  <TableCell>{char}</TableCell>
                  <TableCell>{line}</TableCell>
                  <TableCell>{size}</TableCell>
                </TableRow>
              );
            })}
          </TableBody>
        </Table>
      </TableContainer>
    </Grid>
  );
};

export interface AnalysisDataTableProps {
  results: InvokeAnalyzeResult;
  setResults: Dispatch<SetStateAction<InvokeAnalyzeResult>>;
}

interface DisplayAnalysisData {
  path: string;
  extension: string;
  char: number;
  line: number;
  size: number;
}
