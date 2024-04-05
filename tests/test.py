import pytest
from ms2rescore_rs import get_precursor_info


def test_mgf_precursor_reading():
    precursor = get_precursor_info("tests/data/test.mgf")["peptide1"]
    assert precursor.mz == pytest.approx(475.137295, 0.0001)
    assert precursor.charge == 2
    assert precursor.intensity == 0
    assert precursor.rt == pytest.approx(0.853, 0.001)
    assert precursor.im == 42.42


def test_mzml_precursor_reading():
    precursor = get_precursor_info("tests/data/test.mzML")["index=3"]
    assert precursor.mz == pytest.approx(1007.8454316970522, 0.0001)
    assert precursor.charge == 3
    assert precursor.intensity == 0
    assert precursor.rt == pytest.approx(40.01385810926034, 0.001)
    assert precursor.im == 1.2507906843214256


def test_bruker_tdf_reading():
    precursor = get_precursor_info("tests/data/dda_test.d")["2"]
    assert precursor.mz == pytest.approx(502, 0.0001)
    assert precursor.charge == 2
    assert precursor.intensity == 10
    assert precursor.rt == pytest.approx(0.4, 0.001)
    assert precursor.im == 1.4989212513484358


def test_bruker_minitdf_reading():
    precursor = get_precursor_info("tests/data/test.ms2")["3"]
    assert precursor.mz == pytest.approx(502, 0.0001)
    assert precursor.charge == 2
    assert precursor.intensity == 0
    assert precursor.rt == pytest.approx(0.3, 0.001)
    assert precursor.im == 1.3
