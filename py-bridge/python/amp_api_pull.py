"""Python wrapper for Rust API pull functionality"""
import os
from typing import Dict, Any
import logging

# Import compiled Rust module
try:
    from amp_api_pull import pull_malmo_cleaning_data, process_cleaning_data
except ImportError:
    raise ImportError("Rust module not compiled. Run: cargo build --release")

logger = logging.getLogger(__name__)

class MalmoApiPuller:
    """Wrapper for Rust API pulling functionality"""

    def __init__(self, api_base: str = "https://opendata.malmö.se"):
        self.api_base = api_base
        self.endpoints = {
            "cleaning_schedule": f"{api_base}/api/cleaning",
            "street_data": f"{api_base}/api/streets",
        }

    def pull_cleaning_data(self, output_dir: str) -> Dict[str, Any]:
        """
        Pull data using Rust backend

        Args:
            output_dir: Directory to save parquet files

        Returns:
            Dict with results and metadata
        """
        os.makedirs(output_dir, exist_ok=True)
        output_path = os.path.join(output_dir, "cleaning_schedule.parquet")

        try:
            result = pull_malmo_cleaning_data(
                self.endpoints["cleaning_schedule"],
                output_path
            )
            logger.info(f"✅ Pull successful: {result}")

            return {
                "success": True,
                "message": result,
                "output_path": output_path,
            }
        except Exception as e:
            logger.error(f"❌ Pull failed: {e}")
            return {
                "success": False,
                "error": str(e),
            }

    def process_data(self, json_str: str) -> Dict[str, Any]:
        """Process raw JSON data"""
        try:
            result = process_cleaning_data(json_str)
            return {"success": True, "message": result}
        except Exception as e:
            return {"success": False, "error": str(e)}
