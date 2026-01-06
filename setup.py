from setuptools import setup
from setuptools_rust import Binding, RustExtension

setup(
    name="qars",
    rust_extensions=[
        RustExtension(
            #前缀"valar.valar_core"会生成valar文件夹存放pyd文件
            "qars.qars_core", 
            binding = Binding.PyO3,
            debug = False,
            path = 'Cargo.toml' 
            #sys.path会增加setup.py文件所属目录
        )
    ],
    packages=["qars"],
    # rust extensions are not zip safe, just like C-extensions.
    zip_safe=False,
    include_package_data=True,
    python_requires = ">=3.7",
    requires=['orjson', 'emprical'],
    version="0.0.11"
)
